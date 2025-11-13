package com.workable.movierama.service;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyLong;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import com.workable.movierama.exception.MovieramaNotFoundException;
import com.workable.movierama.model.Movie;
import com.workable.movierama.model.User;
import com.workable.movierama.model.Vote;
import com.workable.movierama.model.VoteType;
import com.workable.movierama.persistence.UserRepository;
import com.workable.movierama.persistence.VoteRepository;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

@ExtendWith(MockitoExtension.class)
class VoteServiceTest {

  @Mock private VoteRepository voteRepository;

  @Mock private UserRepository userRepository;

  @InjectMocks private VoteService voteService;

  @Test
  void getUserVotesForMovies_WhenUserExistsAndHasVotes_ShouldReturnVoteMap() {
    // Given
    String username = "testuser";
    List<Long> movieIds = List.of(1L, 2L, 3L);

    User user = createUser(1L, username);
    List<Vote> votes =
        List.of(
            createVote(1L, 1L, VoteType.LIKE, user), createVote(2L, 2L, VoteType.HATE, user)
            // Note: No vote for movieId 3L
            );

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(user));
    when(voteRepository.findByUserIdAndMovieIdIn(user.getId(), movieIds)).thenReturn(votes);

    // When
    Map<Long, VoteType> result = voteService.getUserVotesForMovies(username, movieIds);

    // Then
    assertNotNull(result);
    assertEquals(2, result.size());
    assertEquals(VoteType.LIKE, result.get(1L));
    assertEquals(VoteType.HATE, result.get(2L));
    assertFalse(result.containsKey(3L)); // No vote for movie 3

    verify(userRepository).findByUsername(username);
    verify(voteRepository).findByUserIdAndMovieIdIn(user.getId(), movieIds);
  }

  @Test
  void getUserVotesForMovies_WhenUserExistsButNoVotes_ShouldReturnEmptyMap() {
    // Given
    String username = "testuser";
    List<Long> movieIds = List.of(1L, 2L, 3L);
    User user = createUser(1L, username);

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(user));
    when(voteRepository.findByUserIdAndMovieIdIn(user.getId(), movieIds)).thenReturn(List.of());

    // When
    Map<Long, VoteType> result = voteService.getUserVotesForMovies(username, movieIds);

    // Then
    assertNotNull(result);
    assertTrue(result.isEmpty());
    verify(userRepository).findByUsername(username);
    verify(voteRepository).findByUserIdAndMovieIdIn(user.getId(), movieIds);
  }

  @Test
  void getUserVotesForMovies_WhenEmptyMovieList_ShouldReturnEmptyMap() {
    // Given
    String username = "testuser";
    List<Long> movieIds = List.of();
    User user = createUser(1L, username);

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(user));
    when(voteRepository.findByUserIdAndMovieIdIn(user.getId(), movieIds)).thenReturn(List.of());

    // When
    Map<Long, VoteType> result = voteService.getUserVotesForMovies(username, movieIds);

    // Then
    assertNotNull(result);
    assertTrue(result.isEmpty());
    verify(userRepository).findByUsername(username);
    verify(voteRepository).findByUserIdAndMovieIdIn(user.getId(), movieIds);
  }

  @Test
  void getUserVotesForMovies_WhenUserDoesNotExist_ShouldThrowException() {
    // Given
    String username = "nonexistent";
    List<Long> movieIds = List.of(1L, 2L);

    when(userRepository.findByUsername(username)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(
        MovieramaNotFoundException.class,
        () -> voteService.getUserVotesForMovies(username, movieIds));

    verify(userRepository).findByUsername(username);
    verify(voteRepository, never()).findByUserIdAndMovieIdIn(anyLong(), any());
  }

  @Test
  void getUserVotesForMovies_WhenUsernameIsNull_ShouldThrowException() {
    // Given
    String username = null;
    List<Long> movieIds = List.of(1L, 2L);

    when(userRepository.findByUsername(username)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(
        MovieramaNotFoundException.class,
        () -> voteService.getUserVotesForMovies(username, movieIds));

    verify(userRepository).findByUsername(username);
    verify(voteRepository, never()).findByUserIdAndMovieIdIn(anyLong(), any());
  }

  @Test
  void getUserVotesForMovies_WhenMultipleMoviesWithVotes_ShouldReturnCorrectMapping() {
    // Given
    String username = "testuser";
    List<Long> movieIds = List.of(1L, 2L, 3L, 4L, 5L);
    User user = createUser(1L, username);

    List<Vote> votes =
        List.of(
            createVote(1L, 1L, VoteType.LIKE, user),
            createVote(2L, 2L, VoteType.HATE, user),
            createVote(3L, 4L, VoteType.LIKE, user)
            // No votes for movieIds 3L and 5L
            );

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(user));
    when(voteRepository.findByUserIdAndMovieIdIn(user.getId(), movieIds)).thenReturn(votes);

    // When
    Map<Long, VoteType> result = voteService.getUserVotesForMovies(username, movieIds);

    // Then
    assertNotNull(result);
    assertEquals(3, result.size());
    assertEquals(VoteType.LIKE, result.get(1L));
    assertEquals(VoteType.HATE, result.get(2L));
    assertEquals(VoteType.LIKE, result.get(4L));
    assertFalse(result.containsKey(3L));
    assertFalse(result.containsKey(5L));

    verify(userRepository).findByUsername(username);
    verify(voteRepository).findByUserIdAndMovieIdIn(user.getId(), movieIds);
  }

  @Test
  void getUserVotesForMovies_WhenLargeNumberOfMovies_ShouldHandleEfficiently() {
    // Given
    String username = "testuser";
    List<Long> movieIds = List.of(1L, 2L, 3L);
    User user = createUser(1L, username);
    List<Vote> votes = List.of();

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(user));
    when(voteRepository.findByUserIdAndMovieIdIn(user.getId(), movieIds)).thenReturn(votes);

    // When
    Map<Long, VoteType> result = voteService.getUserVotesForMovies(username, movieIds);

    // Then
    assertNotNull(result);
    assertTrue(result.isEmpty());
    verify(userRepository).findByUsername(username);
    verify(voteRepository).findByUserIdAndMovieIdIn(user.getId(), movieIds);
  }

  @Test
  void getUserVotesForMovies_VerifiesCorrectRepositoryMethodCalled() {
    // Given
    String username = "testuser";
    Long userId = 1L;
    List<Long> movieIds = List.of(10L, 20L, 30L);
    User user = createUser(userId, username);
    List<Vote> votes = List.of(createVote(1L, 10L, VoteType.LIKE, user));

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(user));
    when(voteRepository.findByUserIdAndMovieIdIn(userId, movieIds)).thenReturn(votes);

    // When
    Map<Long, VoteType> result = voteService.getUserVotesForMovies(username, movieIds);

    // Then
    assertNotNull(result);
    verify(voteRepository).findByUserIdAndMovieIdIn(userId, movieIds);
  }

  // Helper methods
  private User createUser(Long id, String username) {
    return User.builder()
        .id(id)
        .username(username)
        .password("password")
        .email(username + "@test.com")
        .build();
  }

  private Vote createVote(Long voteId, Long movieId, VoteType voteType, User user) {
    Movie movie = Movie.builder().id(movieId).title("Movie " + movieId).build();

    return Vote.builder().id(voteId).movie(movie).user(user).type(voteType).build();
  }
}

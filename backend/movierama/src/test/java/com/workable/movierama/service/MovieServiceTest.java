package com.workable.movierama.service;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyLong;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import com.workable.movierama.exception.MovieramaBaseException;
import com.workable.movierama.exception.MovieramaNotFoundException;
import com.workable.movierama.model.Movie;
import com.workable.movierama.model.User;
import com.workable.movierama.model.Vote;
import com.workable.movierama.model.VoteType;
import com.workable.movierama.persistence.MovieRepository;
import com.workable.movierama.persistence.UserRepository;
import com.workable.movierama.persistence.VoteRepository;
import java.time.LocalDateTime;
import java.util.List;
import java.util.Optional;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageImpl;
import org.springframework.data.domain.Pageable;

@ExtendWith(MockitoExtension.class)
class MovieServiceTest {

  @Mock private MovieRepository movieRepository;

  @Mock private UserRepository userRepository;

  @Mock private VoteRepository voteRepository;

  @Mock private Pageable pageable;

  @InjectMocks private MovieService movieService;

  @Test
  void getAllMovies_ShouldReturnPageOfMovies() {
    // Given
    List<Movie> movies = List.of(createMovie(1L, "Movie 1"), createMovie(2L, "Movie 2"));
    Page<Movie> expectedPage = new PageImpl<>(movies);
    when(movieRepository.findAll(pageable)).thenReturn(expectedPage);

    // When
    Page<Movie> result = movieService.getAllMovies(pageable);

    // Then
    assertNotNull(result);
    assertEquals(2, result.getContent().size());
    verify(movieRepository).findAll(pageable);
  }

  @Test
  void getMoviesByUsername_WhenUserExists_ShouldReturnPageOfMovies() {
    // Given
    String username = "testuser";
    List<Movie> movies = List.of(createMovie(1L, "Movie 1"), createMovie(2L, "Movie 2"));
    Page<Movie> expectedPage = new PageImpl<>(movies);
    when(movieRepository.findByUser_Username(username, pageable)).thenReturn(expectedPage);

    // When
    Page<Movie> result = movieService.getMoviesByUsername(username, pageable);

    // Then
    assertNotNull(result);
    assertEquals(2, result.getContent().size());
    verify(movieRepository).findByUser_Username(username, pageable);
  }

  @Test
  void getMovie_WhenMovieExists_ShouldReturnMovie() {
    // Given
    Long movieId = 1L;
    Movie expectedMovie = createMovie(movieId, "Test Movie");
    when(movieRepository.findById(movieId)).thenReturn(Optional.of(expectedMovie));

    // When
    Movie result = movieService.getMovie(movieId);

    // Then
    assertNotNull(result);
    assertEquals(movieId, result.getId());
    assertEquals("Test Movie", result.getTitle());
    verify(movieRepository).findById(movieId);
  }

  @Test
  void getMovie_WhenMovieDoesNotExist_ShouldThrowException() {
    // Given
    Long movieId = 999L;
    when(movieRepository.findById(movieId)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(MovieramaNotFoundException.class, () -> movieService.getMovie(movieId));
    verify(movieRepository).findById(movieId);
  }

  @Test
  void createMovie_WhenUserExists_ShouldCreateAndReturnMovie() {
    // Given
    String username = "testuser";
    User user = createUser(1L, username);
    Movie inputMovie = createMovie(null, "New Movie");
    Movie expectedMovie = createMovie(1L, "New Movie");
    expectedMovie.setUser(user);
    expectedMovie.setDateAdded(LocalDateTime.now());

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(user));
    when(movieRepository.save(any(Movie.class))).thenReturn(expectedMovie);

    // When
    Movie result = movieService.createMovie(username, inputMovie);

    // Then
    assertNotNull(result);
    assertEquals(1L, result.getId());
    assertEquals(user, result.getUser());
    assertNotNull(result.getDateAdded());
    verify(userRepository).findByUsername(username);
    verify(movieRepository).save(inputMovie);
  }

  @Test
  void createMovie_WhenUserDoesNotExist_ShouldThrowException() {
    // Given
    String username = "nonexistent";
    Movie movie = createMovie(null, "New Movie");
    when(userRepository.findByUsername(username)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(MovieramaNotFoundException.class, () -> movieService.createMovie(username, movie));
    verify(userRepository).findByUsername(username);
    verify(movieRepository, never()).save(any(Movie.class));
  }

  @Test
  void voteMovie_WhenNewVote_ShouldCreateVoteAndReturnMovie() {
    // Given
    String username = "voter";
    Long movieId = 1L;
    VoteType voteType = VoteType.LIKE;

    User voter = createUser(2L, username);
    User movieOwner = createUser(1L, "owner");
    Movie movie = createMovie(movieId, "Test Movie");
    movie.setUser(movieOwner);

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(voter));
    when(movieRepository.findById(movieId)).thenReturn(Optional.of(movie));
    when(voteRepository.findByUserIdAndMovieId(voter.getId(), movieId))
        .thenReturn(Optional.empty());
    when(voteRepository.save(any(Vote.class))).thenAnswer(invocation -> invocation.getArgument(0));

    // When
    Movie result = movieService.voteMovie(username, movieId, voteType);

    // Then
    assertNotNull(result);
    assertEquals(movieId, result.getId());
    verify(voteRepository).findByUserIdAndMovieId(voter.getId(), movieId);
    verify(voteRepository).save(any(Vote.class));
    verify(voteRepository, never()).delete(any(Vote.class));
  }

  @Test
  void voteMovie_WhenExistingSameVote_ShouldRemoveVote() {
    // Given
    String username = "voter";
    Long movieId = 1L;
    VoteType voteType = VoteType.LIKE;

    User voter = createUser(2L, username);
    User movieOwner = createUser(1L, "owner");
    Movie movie = createMovie(movieId, "Test Movie");
    movie.setUser(movieOwner);
    Vote existingVote = new Vote(1L, movie, voter, voteType);

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(voter));
    when(movieRepository.findById(movieId)).thenReturn(Optional.of(movie));
    when(voteRepository.findByUserIdAndMovieId(voter.getId(), movieId))
        .thenReturn(Optional.of(existingVote));

    // When
    Movie result = movieService.voteMovie(username, movieId, voteType);

    // Then
    assertNotNull(result);
    verify(voteRepository).findByUserIdAndMovieId(voter.getId(), movieId);
    verify(voteRepository).delete(existingVote);
    verify(voteRepository, never()).save(any(Vote.class));
  }

  @Test
  void voteMovie_WhenExistingDifferentVote_ShouldUpdateVote() {
    // Given
    String username = "voter";
    Long movieId = 1L;
    VoteType newVoteType = VoteType.HATE;
    VoteType existingVoteType = VoteType.LIKE;

    User voter = createUser(2L, username);
    User movieOwner = createUser(1L, "owner");
    Movie movie = createMovie(movieId, "Test Movie");
    movie.setUser(movieOwner);
    Vote existingVote = new Vote(1L, movie, voter, existingVoteType);

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(voter));
    when(movieRepository.findById(movieId)).thenReturn(Optional.of(movie));
    when(voteRepository.findByUserIdAndMovieId(voter.getId(), movieId))
        .thenReturn(Optional.of(existingVote));

    // When
    Movie result = movieService.voteMovie(username, movieId, newVoteType);

    // Then
    assertNotNull(result);
    assertEquals(newVoteType, existingVote.getType());
    verify(voteRepository).findByUserIdAndMovieId(voter.getId(), movieId);
    verify(voteRepository, never()).delete(any(Vote.class));
    verify(voteRepository, never()).save(any(Vote.class));
  }

  @Test
  void voteMovie_WhenUserVotesOwnMovie_ShouldThrowException() {
    // Given
    String username = "owner";
    Long movieId = 1L;
    VoteType voteType = VoteType.LIKE;

    User owner = createUser(1L, username);
    Movie movie = createMovie(movieId, "Test Movie");
    movie.setUser(owner);

    when(userRepository.findByUsername(username)).thenReturn(Optional.of(owner));
    when(movieRepository.findById(movieId)).thenReturn(Optional.of(movie));

    // When & Then
    assertThrows(
        MovieramaBaseException.class, () -> movieService.voteMovie(username, movieId, voteType));
    verify(voteRepository, never()).findByUserIdAndMovieId(anyLong(), anyLong());
    verify(voteRepository, never()).save(any(Vote.class));
    verify(voteRepository, never()).delete(any(Vote.class));
  }

  @Test
  void voteMovie_WhenUserNotFound_ShouldThrowException() {
    // Given
    String username = "nonexistent";
    Long movieId = 1L;
    VoteType voteType = VoteType.LIKE;

    when(userRepository.findByUsername(username)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(
        MovieramaNotFoundException.class,
        () -> movieService.voteMovie(username, movieId, voteType));
    verify(userRepository).findByUsername(username);
    verify(movieRepository, never()).findById(anyLong());
    verify(voteRepository, never()).findByUserIdAndMovieId(anyLong(), anyLong());
  }

  @Test
  void voteMovie_WhenMovieNotFound_ShouldThrowException() {
    // Given
    String username = "voter";
    Long movieId = 999L;
    VoteType voteType = VoteType.LIKE;

    User voter = createUser(2L, username);
    when(userRepository.findByUsername(username)).thenReturn(Optional.of(voter));
    when(movieRepository.findById(movieId)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(
        MovieramaNotFoundException.class,
        () -> movieService.voteMovie(username, movieId, voteType));
    verify(userRepository).findByUsername(username);
    verify(movieRepository).findById(movieId);
    verify(voteRepository, never()).findByUserIdAndMovieId(anyLong(), anyLong());
  }

  // Helper methods
  private Movie createMovie(Long id, String title) {
    Movie movie = new Movie();
    movie.setId(id);
    movie.setTitle(title);
    movie.setDescription("Test description");
    return movie;
  }

  private User createUser(Long id, String username) {
    User user = new User();
    user.setId(id);
    user.setUsername(username);
    return user;
  }
}

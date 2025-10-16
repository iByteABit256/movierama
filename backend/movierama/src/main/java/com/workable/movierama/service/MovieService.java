package com.workable.movierama.service;

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
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class MovieService {

  private final MovieRepository movieRepository;
  private final UserRepository userRepository;
  private final VoteRepository voteRepository;

  public Page<Movie> getAllMovies(Pageable pageable) {
    return movieRepository.findAll(pageable);
  }

  public Movie createMovie(String username, Movie movie) {
    User user =
        userRepository
            .findByUsername(username)
            .orElseThrow(() -> new MovieramaNotFoundException("User not found"));
    movie.setUser(user);
    movie.setDateAdded(LocalDateTime.now());
    return movieRepository.save(movie);
  }

  public void voteMovie(String username, Long movieId, VoteType type) {
    User user =
        userRepository
            .findByUsername(username)
            .orElseThrow(() -> new MovieramaNotFoundException("User not found"));
    Movie movie =
        movieRepository
            .findById(movieId)
            .orElseThrow(() -> new MovieramaNotFoundException("Movie not found"));

    if (movie.getUser().getId().equals(user.getId()))
      throw new MovieramaBaseException("You cannot vote on your own movie");

    voteRepository
        .findByUserIdAndMovieId(user.getId(), movieId)
        .ifPresentOrElse(
            existing -> {
              if (existing.getType() == type) voteRepository.delete(existing);
              else existing.setType(type);
            },
            () -> voteRepository.save(new Vote(null, movie, user, type)));
  }
}

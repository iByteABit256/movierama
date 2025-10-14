package com.workable.movierama.service;

import com.workable.movierama.model.Movie;
import com.workable.movierama.model.User;
import com.workable.movierama.model.Vote;
import com.workable.movierama.model.VoteType;
import com.workable.movierama.persistence.MovieRepository;
import com.workable.movierama.persistence.UserRepository;
import com.workable.movierama.persistence.VoteRepository;
import java.time.LocalDateTime;
import java.util.List;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class MovieService {

  private final MovieRepository movieRepository;
  private final UserRepository userRepository;
  private final VoteRepository voteRepository;

  public List<Movie> getAllMovies(String sortBy) {
    Sort sort =
        switch (sortBy) {
          case "likes" -> Sort.by(Sort.Direction.DESC, "likesCount");
          case "hates" -> Sort.by(Sort.Direction.DESC, "hatesCount");
          default -> Sort.by(Sort.Direction.DESC, "dateAdded");
        };
    return movieRepository.findAll(sort);
  }

  public Movie createMovie(String username, Movie movie) {
    User user =
        userRepository
            .findByUsername(username)
            .orElseThrow(() -> new RuntimeException("User not found"));
    movie.setUser(user);
    movie.setDateAdded(LocalDateTime.now());
    return movieRepository.save(movie);
  }

  public void voteMovie(String username, Long movieId, VoteType type) {
    User user =
        userRepository
            .findByUsername(username)
            .orElseThrow(() -> new RuntimeException("User not found"));
    Movie movie =
        movieRepository
            .findById(movieId)
            .orElseThrow(() -> new RuntimeException("Movie not found"));

    if (movie.getUser().getId().equals(user.getId()))
      throw new RuntimeException("You cannot vote on your own movie");

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

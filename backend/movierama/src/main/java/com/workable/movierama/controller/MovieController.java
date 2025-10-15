package com.workable.movierama.controller;

import com.workable.movierama.dto.CreateMovieDTO;
import com.workable.movierama.dto.MovieDTO;
import com.workable.movierama.dto.mappers.MovieMapper;
import com.workable.movierama.model.VoteType;
import com.workable.movierama.service.MovieService;
import java.util.List;
import lombok.RequiredArgsConstructor;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1/movies")
@RequiredArgsConstructor
public class MovieController {

  private final MovieService movieService;
  private final MovieMapper movieMapper;

  @GetMapping
  public List<MovieDTO> listMovies(@RequestParam(defaultValue = "date") String sortBy) {
    return movieService.getAllMovies(sortBy).stream().map(movieMapper::entityToDto).toList();
  }

  @PostMapping
  public MovieDTO addMovie(
      @AuthenticationPrincipal UserDetails user, @RequestBody CreateMovieDTO createMovieDto) {
    return movieMapper.entityToDto(
        movieService.createMovie(user.getUsername(), movieMapper.dtoToEntity(createMovieDto)));
  }

  @PostMapping("/{movieId}/vote")
  public void vote(
      @AuthenticationPrincipal UserDetails user,
      @PathVariable Long movieId,
      @RequestParam VoteType type) {
    movieService.voteMovie(user.getUsername(), movieId, type);
  }
}

package com.workable.movierama.controller;

import com.workable.movierama.dto.CreateMovieDTO;
import com.workable.movierama.dto.MovieDTO;
import com.workable.movierama.dto.mappers.MovieMapper;
import com.workable.movierama.model.VoteType;
import com.workable.movierama.service.MovieService;
import java.util.List;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.CrossOrigin;
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
@CrossOrigin(origins = "http://localhost:9000")
public class MovieController {

  private final MovieService movieService;
  private final MovieMapper movieMapper;

  @GetMapping
  public List<MovieDTO> listMovies(@RequestParam(defaultValue = "date") String sortBy) {
    return movieService.getAllMovies(sortBy).stream().map(movieMapper::entityToDto).toList();
  }

  @PostMapping
  public MovieDTO addMovie(
      /*@AuthenticationPrincipal UserDetails user*/ String userName,
      @RequestBody CreateMovieDTO createMovieDto) {
    return movieMapper.entityToDto(
        movieService.createMovie(userName, movieMapper.dtoToEntity(createMovieDto)));
  }

  @PostMapping("/{movieId}/vote")
  public void vote(
      /*@AuthenticationPrincipal UserDetails user*/ String userName,
      @PathVariable Long movieId,
      @RequestParam VoteType type) {
    movieService.voteMovie(userName, movieId, type);
  }
}

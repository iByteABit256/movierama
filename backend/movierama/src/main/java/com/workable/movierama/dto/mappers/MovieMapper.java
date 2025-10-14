package com.workable.movierama.dto.mappers;

import com.workable.movierama.dto.CreateMovieDTO;
import com.workable.movierama.dto.MovieDTO;
import com.workable.movierama.model.Movie;
import com.workable.movierama.model.VoteType;
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

@Mapper(componentModel = "spring", imports = VoteType.class)
public interface MovieMapper {
  @Mapping(target = "username", expression = "java(movie.getUser().getUsername())")
  @Mapping(
      target = "likes",
      expression =
          "java(movie.getVotes() == null ? 0L : movie.getVotes().stream().filter(v -> v.getType()"
              + " == VoteType.LIKE).count())")
  @Mapping(
      target = "hates",
      expression =
          "java(movie.getVotes() == null ? 0L : movie.getVotes().stream().filter(v -> v.getType()"
              + " == VoteType.HATE).count())")
  public MovieDTO entityToDto(Movie movie);

  @Mapping(target = "id", ignore = true)
  @Mapping(target = "user", ignore = true)
  @Mapping(target = "dateAdded", expression = "java(java.time.LocalDateTime.now())")
  @Mapping(target = "votes", ignore = true)
  public Movie dtoToEntity(CreateMovieDTO movieDto);
}

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
  @Mapping(target = "likes", source = "likeCount")
  @Mapping(target = "hates", source = "hateCount")
  public MovieDTO entityToDto(Movie movie);

  @Mapping(target = "id", ignore = true)
  @Mapping(target = "user", ignore = true)
  @Mapping(target = "dateAdded", expression = "java(java.time.LocalDateTime.now())")
  @Mapping(target = "votes", ignore = true)
  @Mapping(target = "likeCount", ignore = true)
  @Mapping(target = "hateCount", ignore = true)
  public Movie dtoToEntity(CreateMovieDTO movieDto);
}

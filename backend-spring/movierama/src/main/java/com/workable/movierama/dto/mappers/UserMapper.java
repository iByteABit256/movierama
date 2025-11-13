package com.workable.movierama.dto.mappers;

import com.workable.movierama.dto.UserDTO;
import com.workable.movierama.model.User;
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

@Mapper(componentModel = "spring")
public interface UserMapper {
  public UserDTO entityToDto(User entity);

  @Mapping(target = "password", ignore = true)
  @Mapping(target = "movies", ignore = true)
  public User dtoToEntity(UserDTO entity);
}

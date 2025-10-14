package com.workable.movierama.service;

import com.workable.movierama.dto.RegisterUserDTO;
import com.workable.movierama.dto.UserDTO;
import com.workable.movierama.dto.mappers.UserMapper;
import com.workable.movierama.model.User;
import com.workable.movierama.persistence.UserRepository;
import java.util.List;
import java.util.stream.Collectors;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class UserService {

  private final UserRepository userRepository;
  private final UserMapper userMapper;

  public List<UserDTO> getAllUsers() {
    return userRepository.findAll().stream()
        .map(userMapper::entityToDto)
        .collect(Collectors.toList());
  }

  public UserDTO getUserById(Long id) {
    return userRepository
        .findById(id)
        .map(userMapper::entityToDto)
        .orElseThrow(() -> new IllegalArgumentException("User not found"));
  }

  public UserDTO registerUser(RegisterUserDTO dto) {
    if (userRepository.findByUsername(dto.username()).isPresent()) {
      throw new IllegalArgumentException("Username already exists");
    }

    User user =
        User.builder()
            .username(dto.username())
            .password(dto.password()) // ⚠️ You’ll hash this once security is added
            .build();

    userRepository.save(user);
    return userMapper.entityToDto(user);
  }
}

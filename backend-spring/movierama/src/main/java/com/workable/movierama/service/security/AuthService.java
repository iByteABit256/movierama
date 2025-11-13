package com.workable.movierama.service.security;

import com.workable.movierama.dto.AuthRequestDTO;
import com.workable.movierama.dto.AuthResponseDTO;
import com.workable.movierama.dto.RegisterUserDTO;
import com.workable.movierama.exception.MovieramaBaseException;
import com.workable.movierama.model.User;
import com.workable.movierama.persistence.UserRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class AuthService {

  private final UserRepository userRepository;
  private final PasswordEncoder passwordEncoder;
  private final JwtService jwtService;

  public AuthResponseDTO register(RegisterUserDTO dto) {
    if (userRepository.findByUsername(dto.username()).isPresent()) {
      throw new MovieramaBaseException("Username already exists");
    }

    var user =
        User.builder()
            .username(dto.username())
            .email(dto.email())
            .password(passwordEncoder.encode(dto.password()))
            .build();
    userRepository.save(user);

    return new AuthResponseDTO(jwtService.generateToken(user.getUsername()));
  }

  public AuthResponseDTO login(AuthRequestDTO request) {
    var user =
        userRepository
            .findByUsername(request.username())
            .orElseThrow(() -> new MovieramaBaseException("Invalid credentials"));

    if (!passwordEncoder.matches(request.password(), user.getPassword())) {
      throw new MovieramaBaseException("Invalid credentials");
    }

    return new AuthResponseDTO(jwtService.generateToken(user.getUsername()));
  }
}

package com.workable.movierama.controller;

import com.workable.movierama.dto.AuthRequestDTO;
import com.workable.movierama.dto.AuthResponseDTO;
import com.workable.movierama.dto.RegisterUserDTO;
import com.workable.movierama.service.security.AuthService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1/auth")
@RequiredArgsConstructor
public class AuthController {

  private final AuthService authService;

  @PostMapping("/register")
  public AuthResponseDTO register(@RequestBody RegisterUserDTO dto) {
    return authService.register(dto);
  }

  @PostMapping("/login")
  public AuthResponseDTO login(@RequestBody AuthRequestDTO request) {
    return authService.login(request);
  }
}

package com.workable.movierama.controller;

import com.workable.movierama.dto.RegisterUserDTO;
import com.workable.movierama.dto.UserDTO;
import com.workable.movierama.service.UserService;
import java.util.List;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/v1/users")
@RequiredArgsConstructor
public class UserController {

  private final UserService userService;

  @GetMapping
  public ResponseEntity<List<UserDTO>> getAllUsers() {
    return ResponseEntity.ok(userService.getAllUsers());
  }

  @GetMapping("/{id}")
  public ResponseEntity<UserDTO> getUserById(@PathVariable Long id) {
    return ResponseEntity.ok(userService.getUserById(id));
  }

  @PostMapping("/register")
  public ResponseEntity<UserDTO> register(@RequestBody RegisterUserDTO dto) {
    return ResponseEntity.ok(userService.registerUser(dto));
  }
}

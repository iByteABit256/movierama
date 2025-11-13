package com.workable.movierama.service;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.times;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import com.workable.movierama.dto.RegisterUserDTO;
import com.workable.movierama.dto.UserDTO;
import com.workable.movierama.dto.mappers.UserMapper;
import com.workable.movierama.exception.MovieramaBaseException;
import com.workable.movierama.exception.MovieramaNotFoundException;
import com.workable.movierama.model.User;
import com.workable.movierama.persistence.UserRepository;
import java.util.List;
import java.util.Optional;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

@ExtendWith(MockitoExtension.class)
class UserServiceTest {

  @Mock private UserRepository userRepository;

  @Mock private UserMapper userMapper;

  @InjectMocks private UserService userService;

  @Test
  void getAllUsers_WhenUsersExist_ShouldReturnListOfUserDTOs() {
    // Given
    List<User> users =
        List.of(
            createUser(1L, "user1", "user1@test.com"), createUser(2L, "user2", "user2@test.com"));

    List<UserDTO> expectedDTOs =
        List.of(
            createUserDTO(1L, "user1", "user1@test.com"),
            createUserDTO(2L, "user2", "user2@test.com"));

    when(userRepository.findAll()).thenReturn(users);
    when(userMapper.entityToDto(users.get(0))).thenReturn(expectedDTOs.get(0));
    when(userMapper.entityToDto(users.get(1))).thenReturn(expectedDTOs.get(1));

    // When
    List<UserDTO> result = userService.getAllUsers();

    // Then
    assertNotNull(result);
    assertEquals(2, result.size());
    assertEquals(expectedDTOs, result);
    verify(userRepository).findAll();
    verify(userMapper, times(2)).entityToDto(any(User.class));
  }

  @Test
  void getAllUsers_WhenNoUsersExist_ShouldReturnEmptyList() {
    // Given
    when(userRepository.findAll()).thenReturn(List.of());

    // When
    List<UserDTO> result = userService.getAllUsers();

    // Then
    assertNotNull(result);
    assertTrue(result.isEmpty());
    verify(userRepository).findAll();
    verify(userMapper, never()).entityToDto(any(User.class));
  }

  @Test
  void getUserById_WhenUserExists_ShouldReturnUserDTO() {
    // Given
    Long userId = 1L;
    User user = createUser(userId, "testuser", "test@test.com");
    UserDTO expectedDTO = createUserDTO(userId, "testuser", "test@test.com");

    when(userRepository.findById(userId)).thenReturn(Optional.of(user));
    when(userMapper.entityToDto(user)).thenReturn(expectedDTO);

    // When
    UserDTO result = userService.getUserById(userId);

    // Then
    assertNotNull(result);
    assertEquals(userId, result.id());
    assertEquals("testuser", result.username());
    assertEquals("test@test.com", result.email());
    verify(userRepository).findById(userId);
    verify(userMapper).entityToDto(user);
  }

  @Test
  void getUserById_WhenUserDoesNotExist_ShouldThrowException() {
    // Given
    Long userId = 999L;
    when(userRepository.findById(userId)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(MovieramaNotFoundException.class, () -> userService.getUserById(userId));
    verify(userRepository).findById(userId);
    verify(userMapper, never()).entityToDto(any(User.class));
  }

  @Test
  void getUserById_WhenUserIdIsNull_ShouldThrowException() {
    // Given
    Long userId = null;
    when(userRepository.findById(userId)).thenReturn(Optional.empty());

    // When & Then
    assertThrows(MovieramaNotFoundException.class, () -> userService.getUserById(userId));
    verify(userRepository).findById(userId);
    verify(userMapper, never()).entityToDto(any(User.class));
  }

  @Test
  void registerUser_WhenValidNewUser_ShouldCreateAndReturnUserDTO() {
    // Given
    RegisterUserDTO registerDTO = new RegisterUserDTO("newuser", "password123", "newuser@test.com");
    User savedUser = createUser(null, "newuser", "newuser@test.com");
    UserDTO expectedDTO = createUserDTO(1L, "newuser", "newuser@test.com");

    when(userRepository.findByUsername("newuser")).thenReturn(Optional.empty());
    when(userRepository.save(any(User.class))).thenReturn(savedUser);
    when(userMapper.entityToDto(savedUser)).thenReturn(expectedDTO);

    // When
    UserDTO result = userService.registerUser(registerDTO);

    // Then
    assertNotNull(result);
    assertEquals(1L, result.id());
    assertEquals("newuser", result.username());
    assertEquals("newuser@test.com", result.email());

    verify(userRepository).findByUsername("newuser");
    verify(userRepository).save(any(User.class));
    verify(userMapper).entityToDto(savedUser);
  }

  @Test
  void registerUser_WhenUsernameAlreadyExists_ShouldThrowException() {
    // Given
    RegisterUserDTO registerDTO =
        new RegisterUserDTO("existinguser", "password123", "existing@test.com");
    User existingUser = createUser(1L, "existinguser", "existing@test.com");

    when(userRepository.findByUsername("existinguser")).thenReturn(Optional.of(existingUser));

    // When & Then
    assertThrows(MovieramaBaseException.class, () -> userService.registerUser(registerDTO));

    verify(userRepository).findByUsername("existinguser");
    verify(userRepository, never()).save(any(User.class));
    verify(userMapper, never()).entityToDto(any(User.class));
  }

  // Helper methods
  private User createUser(Long id, String username, String email) {
    return User.builder().id(id).username(username).password("password123").email(email).build();
  }

  private UserDTO createUserDTO(Long id, String username, String email) {
    return new UserDTO(id, username, email);
  }
}

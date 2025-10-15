package com.workable.movierama.exception;

import com.workable.movierama.dto.ErrorDTO;
import jakarta.servlet.http.HttpServletRequest;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

@RestControllerAdvice
public class MovieramaExceptionHandler {

  @ExceptionHandler(MovieramaNotFoundException.class)
  public ResponseEntity<ErrorDTO> handleNotFound(
      MovieramaNotFoundException ex, HttpServletRequest request) {

    return ResponseEntity.status(HttpStatus.NOT_FOUND).body(new ErrorDTO(ex.getMessage()));
  }

  @ExceptionHandler(MovieramaBaseException.class)
  public ResponseEntity<ErrorDTO> handleBase(
      MovieramaBaseException ex, HttpServletRequest request) {

    return ResponseEntity.status(HttpStatus.BAD_REQUEST).body(new ErrorDTO(ex.getMessage()));
  }

  @ExceptionHandler(Exception.class)
  public ResponseEntity<ErrorDTO> handleGeneric(Exception ex, HttpServletRequest request) {

    return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR)
        .body(new ErrorDTO(ex.getMessage()));
  }
}

package com.workable.movierama.exception;

import com.workable.movierama.dto.ErrorDTO;
import org.springframework.data.mapping.PropertyReferenceException;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

@RestControllerAdvice
public class MovieramaExceptionHandler {

  @ExceptionHandler(MovieramaNotFoundException.class)
  public ResponseEntity<ErrorDTO> handleNotFound(MovieramaNotFoundException ex) {

    return ResponseEntity.status(HttpStatus.NOT_FOUND).body(new ErrorDTO(ex.getMessage()));
  }

  @ExceptionHandler({
    MovieramaBaseException.class,
    PropertyReferenceException.class,
    IllegalArgumentException.class
  })
  public ResponseEntity<ErrorDTO> handleBadRequest(Exception ex) {
    return ResponseEntity.badRequest().body(new ErrorDTO(ex.getMessage()));
  }
}

package com.workable.movierama.dto;

import com.fasterxml.jackson.annotation.JsonFormat;
import java.time.LocalDateTime;

public record MovieDTO(
    Long id,
    String title,
    String description,
    String username,
    @JsonFormat(pattern = "yyyy-MM-dd'T'HH:mm:ss") LocalDateTime dateAdded,
    long likes,
    long hates) {}

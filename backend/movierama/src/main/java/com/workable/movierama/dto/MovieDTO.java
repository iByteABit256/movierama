package com.workable.movierama.dto;

import java.time.LocalDateTime;

public record MovieDTO(
    Long id,
    String title,
    String description,
    String username,
    LocalDateTime dateAdded,
    long likes,
    long hates) {}

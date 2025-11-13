package com.workable.movierama.controller;

import com.workable.movierama.model.VoteType;
import com.workable.movierama.service.VoteService;
import java.util.List;
import java.util.Map;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1/votes")
@RequiredArgsConstructor
public class VoteController {

  private final VoteService voteService;

  @PostMapping("/user-votes")
  public ResponseEntity<Map<Long, VoteType>> getUserVotesForMovies(
      @RequestBody List<Long> movieIds, @AuthenticationPrincipal UserDetails user) {
    final Map<Long, VoteType> votes =
        voteService.getUserVotesForMovies(user.getUsername(), movieIds);
    return ResponseEntity.ok(votes);
  }
}

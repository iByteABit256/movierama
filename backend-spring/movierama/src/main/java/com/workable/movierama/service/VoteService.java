package com.workable.movierama.service;

import com.workable.movierama.exception.MovieramaNotFoundException;
import com.workable.movierama.model.User;
import com.workable.movierama.model.Vote;
import com.workable.movierama.model.VoteType;
import com.workable.movierama.persistence.UserRepository;
import com.workable.movierama.persistence.VoteRepository;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class VoteService {

  private final VoteRepository voteRepository;
  private final UserRepository userRepository;

  public Map<Long, VoteType> getUserVotesForMovies(String username, List<Long> movieIds) {
    final User user =
        userRepository
            .findByUsername(username)
            .orElseThrow(() -> new MovieramaNotFoundException("User not found"));

    final List<Vote> votes = voteRepository.findByUserIdAndMovieIdIn(user.getId(), movieIds);

    return votes.stream().collect(Collectors.toMap(vote -> vote.getMovie().getId(), Vote::getType));
  }
}

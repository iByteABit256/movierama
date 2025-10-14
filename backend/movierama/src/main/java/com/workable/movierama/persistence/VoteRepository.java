package com.workable.movierama.persistence;

import com.workable.movierama.model.Vote;
import com.workable.movierama.model.VoteType;
import java.util.Optional;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface VoteRepository extends JpaRepository<Vote, Long> {
  Optional<Vote> findByUserIdAndMovieId(Long userId, Long movieId);

  long countByMovieIdAndType(Long movieId, VoteType type);
}

package com.workable.movierama.persistence;

import com.workable.movierama.model.Movie;
import java.util.List;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface MovieRepository extends JpaRepository<Movie, Long> {
  List<Movie> findByUserId(Long userId);
}

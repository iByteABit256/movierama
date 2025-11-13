package com.workable.movierama.persistence;

import com.workable.movierama.model.Movie;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface MovieRepository extends JpaRepository<Movie, Long> {
  Page<Movie> findByUser_Username(String username, Pageable pageable);
}

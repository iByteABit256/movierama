package com.workable.movierama;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.data.jpa.repository.config.EnableJpaRepositories;

@SpringBootApplication
@EnableJpaRepositories(basePackages = "com.workable.movierama.persistence")
public class MovieramaApplication {

  public static void main(String[] args) {
    SpringApplication.run(MovieramaApplication.class, args);
  }
}

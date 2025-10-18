package com.workable.movierama.config;

import com.workable.movierama.dto.RegisterUserDTO;
import com.workable.movierama.model.Movie;
import com.workable.movierama.model.User;
import com.workable.movierama.model.Vote;
import com.workable.movierama.model.VoteType;
import com.workable.movierama.persistence.MovieRepository;
import com.workable.movierama.persistence.UserRepository;
import com.workable.movierama.persistence.VoteRepository;
import com.workable.movierama.service.security.AuthService;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.boot.CommandLineRunner;
import org.springframework.context.annotation.Profile;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Component;

import java.time.LocalDateTime;
import java.util.List;
import java.util.Random;
import java.util.stream.IntStream;

@Component
@Profile("dev") // only runs in dev profile
@RequiredArgsConstructor
@Slf4j
public class MockDataInitializer implements CommandLineRunner {

    private final AuthService authService;
    private final UserRepository userRepository;
    private final MovieRepository movieRepository;
    private final VoteRepository voteRepository;

    private final Random random = new Random();

    @Override
    public void run(String... args) {
        if (movieRepository.count() > 0) {
            return; // Skip if data already exists
        }

        // Create users
        IntStream.rangeClosed(1, 5)
                .mapToObj(i -> new RegisterUserDTO(
                        "user" + i, "password", "user" + i + "@mail.com"))
                .forEach(authService::register);

        List<User> users = userRepository.findAll();

        // Create movies
        List<Movie> movies = IntStream.rangeClosed(1, 30)
                .mapToObj(i -> Movie.builder()
                        .title("Movie " + i)
                        .description("Description for Movie " + i)
                        .dateAdded(LocalDateTime.now().minusDays(random.nextInt(10)))
                        .user(users.get(random.nextInt(users.size())))
                        .build())
                .toList();
        saveAllEntities(movies, movieRepository, "movie");

        // Create votes
        List<Vote> votes = IntStream.rangeClosed(1, 100)
                .mapToObj(i -> Vote.builder()
                        .type(random.nextBoolean() ? VoteType.LIKE : VoteType.HATE)
                        .user(users.get(random.nextInt(users.size())))
                        .movie(movies.get(random.nextInt(movies.size())))
                        .build())
                .toList();
        saveAllEntities(votes, voteRepository, "vote");

        log.info("Mock data initialized successfully!");
    }

    private <T> void saveAllEntities(List<T> entities, JpaRepository<T, ?> repository, String entityName) {
        for (T entity : entities) {
            try {
                repository.save(entity);
            } catch (Exception e) {
                log.warn("Skipped {} creation because of constraint violation", entityName);
            }
        }
    }
}
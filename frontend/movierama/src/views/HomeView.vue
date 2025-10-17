<script setup>
import { onMounted } from "vue";
import MovieCard from "../components/MovieCard.vue";
import { useMoviesStore } from "../store/movies";

const moviesStore = useMoviesStore();

onMounted(() => moviesStore.fetchMovies());
</script>

<template>
  <div class="home">
    <MovieCard
      v-for="m in moviesStore.movies"
      :key="m.id"
      :movie="m"
      @vote="moviesStore.vote"
    />
  </div>
</template>

<style scoped>
.home {
  max-width: 800px;
  margin: 2rem auto;
}
.filters {
  margin-bottom: 1rem;
}
</style>

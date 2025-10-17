<template>
  <div class="movie-card">
    <h3>{{ movie.title }}</h3>
    <p>{{ movie.description }}</p>
    <p class="meta">
      Posted by
      <router-link :to="`/user/${movie.username}`">{{ movie.username }}</router-link>
      on {{ new Date(movie.dateAdded).toLocaleDateString() }}
    </p>

    <div class="votes">
      <button>👍 {{ movie.likes }}</button>
      <button>👎 {{ movie.hates }}</button>
    </div>
  </div>
</template>

<script setup>
defineProps({
  movie: {
    type: Object,
    required: true,
  },
})
</script>

<style scoped>
.movie-card {
  border: 1px solid #ddd;
  padding: 1rem;
  border-radius: 10px;
  margin-bottom: 1rem;
  background: #fafafa;
}
.meta {
  color: #666;
  font-size: 0.9rem;
}
.votes button {
  margin-right: 0.5rem;
  background: #eee;
  border: none;
  padding: 0.3rem 0.6rem;
  cursor: pointer;
}
.votes button:hover {
  background: #ddd;
}
</style>

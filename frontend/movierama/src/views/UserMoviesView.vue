<template>
  <MoviesList :username="route.params.username" />
</template>

<script setup>
import { useRoute } from 'vue-router'
import MoviesList from '../components/MoviesList.vue'

const route = useRoute()
</script>

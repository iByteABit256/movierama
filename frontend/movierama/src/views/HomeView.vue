<template>
  <div class="movies-page">
    <h1>All Movies</h1>

    <div class="pagination-controls">
      <label for="pageSize">Movies per page:</label>
      <select
        id="pageSize"
        v-model="pageSize"
        @change="onPageSizeChange"
        :disabled="moviesStore.loading"
      >
        <option value="5">5</option>
        <option value="10">10</option>
        <option value="20">20</option>
        <option value="50">50</option>
      </select>
    </div>

    <div v-if="moviesStore.loading" class="loading">Loading movies...</div>

    <div v-else-if="movies.length === 0" class="no-movies">
      No movies found. Be the first to add one!
    </div>

    <div v-else>
      <div class="movies-list">
        <MovieCard v-for="movie in movies" :key="movie.id" :movie="movie" />
      </div>

      <!-- Pagination -->
      <div class="pagination">
        <div class="pagination-info">
          Showing {{ showingStart }}-{{ showingEnd }} of {{ moviesStore.totalElements }} movies
          (Page {{ moviesStore.currentPage + 1 }} of {{ moviesStore.totalPages }})
        </div>

        <div class="pagination-buttons">
          <button
            @click="goToFirstPage"
            :disabled="!moviesStore.hasPrevPage || moviesStore.loading"
            class="pagination-btn"
          >
            ⏮ First
          </button>
          <button
            @click="prevPage"
            :disabled="!moviesStore.hasPrevPage || moviesStore.loading"
            class="pagination-btn"
          >
            ◀ Previous
          </button>

          <div class="page-numbers">
            <button
              v-for="page in visiblePages"
              :key="page"
              @click="goToPage(page - 1)"
              :class="{
                'pagination-btn': true,
                active: page - 1 === moviesStore.currentPage,
              }"
              :disabled="moviesStore.loading"
            >
              {{ page }}
            </button>
            <span v-if="showEllipsis" class="ellipsis">...</span>
          </div>

          <button
            @click="nextPage"
            :disabled="!moviesStore.hasNextPage || moviesStore.loading"
            class="pagination-btn"
          >
            Next ▶
          </button>
          <button
            @click="goToLastPage"
            :disabled="!moviesStore.hasNextPage || moviesStore.loading"
            class="pagination-btn"
          >
            Last ⏭
          </button>
        </div>
      </div>
    </div>

    <div v-if="moviesStore.error" class="error-message">
      {{ moviesStore.error }}
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useMoviesStore } from '../store/movies'
import MovieCard from '../components/MovieCard.vue'

const moviesStore = useMoviesStore()
const movies = ref([])
const pageSize = ref(10)

const showingStart = computed(() => {
  return moviesStore.currentPage * moviesStore.pageSize + 1
})

const showingEnd = computed(() => {
  const end = (moviesStore.currentPage + 1) * moviesStore.pageSize
  return Math.min(end, moviesStore.totalElements)
})

const visiblePages = computed(() => {
  const current = moviesStore.currentPage
  const total = moviesStore.totalPages
  const pages = []

  // Show max 5 pages around current page
  let start = Math.max(0, current - 2)
  let end = Math.min(total, start + 5)

  // Adjust start if we're near the end
  if (end - start < 5) {
    start = Math.max(0, end - 5)
  }

  for (let i = start; i < end; i++) {
    pages.push(i + 1)
  }

  return pages
})

const showEllipsis = computed(() => {
  return moviesStore.totalPages > visiblePages.value.length
})

const onPageSizeChange = () => {
  moviesStore.setPageSize(parseInt(pageSize.value))
  loadMovies()
}

const loadMovies = async () => {
  try {
    const data = await moviesStore.fetchMovies(moviesStore.currentPage, moviesStore.pageSize)
    movies.value = data.content || []
  } catch (error) {
    console.error('Failed to load movies:', error)
    movies.value = []
  }
}

const nextPage = async () => {
  await moviesStore.nextPage()
  movies.value = moviesStore.movies
}

const prevPage = async () => {
  await moviesStore.prevPage()
  movies.value = moviesStore.movies
}

const goToPage = async (page) => {
  await moviesStore.goToPage(page)
  movies.value = moviesStore.movies
}

const goToFirstPage = async () => {
  await moviesStore.goToPage(0)
  movies.value = moviesStore.movies
}

const goToLastPage = async () => {
  await moviesStore.goToPage(moviesStore.totalPages - 1)
  movies.value = moviesStore.movies
}

onMounted(() => {
  loadMovies()
})

// Watch for page size changes in store
watch(
  () => moviesStore.pageSize,
  (newSize) => {
    pageSize.value = newSize
  },
)
</script>

<style scoped>
@import '../styles/common.css';
@import '../styles/pagination.css';

.movies-page {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}
</style>

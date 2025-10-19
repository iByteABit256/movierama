<template>
  <div class="movies-list-container">
    <!-- Error message with fade -->
    <transition name="fade">
      <div v-if="showError" class="error-message">
        {{ moviesStore.error }}
      </div>
    </transition>

    <h1 v-if="!username">All Movies</h1>
    <h1 v-else>Movies by {{ username }}</h1>

    <!-- Sorting -->
    <div class="sort-controls">
      <label for="sortField">Sort by:</label>
      <select
        id="sortField"
        v-model="sortField"
        @change="onSortChange"
        :disabled="moviesStore.loading"
      >
        <option value="likeCount">Likes</option>
        <option value="hateCount">Hates</option>
        <option value="dateAdded">Date Added</option>
      </select>

      <select
        id="sortDir"
        v-model="sortDirection"
        @change="onSortChange"
        :disabled="moviesStore.loading"
      >
        <option value="desc">Descending</option>
        <option value="asc">Ascending</option>
      </select>
    </div>

    <!-- Page Size -->
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

    <!-- Loading / No data -->
    <div v-if="moviesStore.loading" class="loading">Loading movies...</div>

    <div v-else-if="movieList.length === 0" class="no-movies">
      {{ noMoviesMessage }}
    </div>

    <!-- Movies -->
    <div v-else>
      <div class="movies-list">
        <MovieCard v-for="movie in movieList" :key="movie.id" :movie="movie" />
      </div>

      <!-- Pagination -->
      <div class="pagination">
        <div class="pagination-info">
          Showing {{ showingStart }}-{{ showingEnd }} of {{ totalElements }} movies (Page
          {{ currentPage + 1 }} of {{ totalPages }})
        </div>

        <div class="pagination-buttons">
          <button
            @click="goToFirstPage"
            :disabled="!hasPrevPage || moviesStore.loading"
            class="pagination-btn"
          >
            ⏮ First
          </button>
          <button
            @click="prevPage"
            :disabled="!hasPrevPage || moviesStore.loading"
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
                active: page - 1 === currentPage,
              }"
              :disabled="moviesStore.loading"
            >
              {{ page }}
            </button>
            <span v-if="showEllipsis" class="ellipsis">...</span>
          </div>

          <button
            @click="nextPage"
            :disabled="!hasNextPage || moviesStore.loading"
            class="pagination-btn"
          >
            Next ▶
          </button>
          <button
            @click="goToLastPage"
            :disabled="!hasNextPage || moviesStore.loading"
            class="pagination-btn"
          >
            Last ⏭
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useMoviesStore } from '../store/movies'
import MovieCard from './MovieCard.vue'

const props = defineProps({
  username: {
    type: String,
    default: null,
  },
})

const moviesStore = useMoviesStore()
const pageSize = ref(10)
const sortField = ref(moviesStore.sort.split(',')[0] || 'dateAdded')
const sortDirection = ref(moviesStore.sort.split(',')[1] || 'desc')

// Error fade handling
const showError = ref(false)
let errorTimer = null

watch(
  () => moviesStore.error,
  (newError) => {
    clearTimeout(errorTimer)
    if (newError) {
      showError.value = true
      errorTimer = setTimeout(() => {
        showError.value = false
        moviesStore.clearError()
      }, 2500)
    } else {
      showError.value = false
    }
  },
)

const isUserView = computed(() => !!props.username)

const movieList = computed(() => (isUserView.value ? moviesStore.userMovies : moviesStore.movies))

const totalElements = computed(() =>
  isUserView.value ? moviesStore.userTotalElements : moviesStore.totalElements,
)
const totalPages = computed(() =>
  isUserView.value ? moviesStore.userTotalPages : moviesStore.totalPages,
)
const currentPage = computed(() =>
  isUserView.value ? moviesStore.userCurrentPage : moviesStore.currentPage,
)
const hasNextPage = computed(() =>
  isUserView.value ? moviesStore.hasNextUserPage : moviesStore.hasNextPage,
)
const hasPrevPage = computed(() =>
  isUserView.value ? moviesStore.hasPrevUserPage : moviesStore.hasPrevPage,
)

const showingStart = computed(() => currentPage.value * moviesStore.pageSize + 1)
const showingEnd = computed(() => {
  const end = (currentPage.value + 1) * moviesStore.pageSize
  return Math.min(end, totalElements.value)
})
const noMoviesMessage = computed(() =>
  props.username ? 'No movies found for this user.' : 'No movies found. Be the first to add one!',
)

const visiblePages = computed(() => {
  const current = currentPage.value
  const total = totalPages.value
  const pages = []
  let start = Math.max(0, current - 2)
  let end = Math.min(total, start + 5)
  if (end - start < 5) start = Math.max(0, end - 5)
  for (let i = start; i < end; i++) pages.push(i + 1)
  return pages
})

const showEllipsis = computed(() => totalPages.value > visiblePages.value.length)

const onSortChange = () => {
  moviesStore.sort = `${sortField.value},${sortDirection.value}`
  loadMovies()
}

const onPageSizeChange = () => {
  moviesStore.setPageSize(parseInt(pageSize.value))
  loadMovies()
}

const nextPage = async () => {
  if (isUserView.value) await moviesStore.nextUserPage(props.username)
  else await moviesStore.nextPage()
}

const prevPage = async () => {
  if (isUserView.value) await moviesStore.prevUserPage(props.username)
  else await moviesStore.prevPage()
}

const goToPage = async (page) => {
  if (isUserView.value) await moviesStore.goToUserPage(props.username, page)
  else await moviesStore.goToPage(page)
}

const goToFirstPage = async () => goToPage(0)
const goToLastPage = async () => goToPage(totalPages.value - 1)

const loadMovies = async () => {
  try {
    if (isUserView.value) {
      await moviesStore.fetchMoviesByUser(props.username, currentPage.value, moviesStore.pageSize)
    } else {
      await moviesStore.fetchMovies(currentPage.value, moviesStore.pageSize)
    }
  } catch (error) {
    console.error('Failed to load movies:', error)
  }
}

onMounted(loadMovies)

watch(
  () => props.username,
  () => {
    moviesStore.clearUserVotes()
    loadMovies()
  },
)

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

.movies-list-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}
</style>

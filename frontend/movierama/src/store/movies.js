import { defineStore } from 'pinia'
import api from '../api/api'

export const useMoviesStore = defineStore('movies', {
  state: () => ({
    movies: [],
    currentMovie: null,
    userMovies: [],
    userVotes: new Map(),
    sort: 'dateAdded,desc',
    voting: false,
    loading: false,
    error: null,
    // Pagination state
    currentPage: 0,
    pageSize: 10,
    totalPages: 0,
    totalElements: 0,
    // User movies pagination
    userCurrentPage: 0,
    userTotalPages: 0,
    userTotalElements: 0,
  }),

  getters: {
    getUserVote: (state) => (movieId) => {
      return state.userVotes.get(movieId) || null
    },
    hasNextPage: (state) => {
      return state.currentPage < state.totalPages - 1
    },
    hasPrevPage: (state) => {
      return state.currentPage > 0
    },
    hasNextUserPage: (state) => {
      return state.userCurrentPage < state.userTotalPages - 1
    },
    hasPrevUserPage: (state) => {
      return state.userCurrentPage > 0
    },
    sortParam: (state) => {
      return `${state.sort}`
    },
  },

  actions: {
    async fetchMovies(page = 0, size = 10) {
      this.loading = true
      this.error = null
      try {
        const { data } = await api.get('/movies', {
          params: {
            page,
            size,
            sort: this.sort,
          },
        })
        this.movies = data.content || []
        this.currentPage = data.number || 0
        this.totalPages = data.totalPages || 0
        this.totalElements = data.totalElements || 0
        this.pageSize = data.size || size

        // Fetch user votes for the visible movies
        if (this.movies.length > 0) {
          await this.fetchUserVotesForMovies(this.movies)
        }

        return data
      } catch (error) {
        this.error = error.response?.data?.message || 'Failed to fetch movies'
        throw error
      } finally {
        this.loading = false
      }
    },

    async fetchMovieById(movieId) {
      this.loading = true
      this.error = null
      try {
        const { data } = await api.get(`/movies/${movieId}`)
        this.currentMovie = data

        // Fetch user vote for this single movie
        await this.fetchUserVotesForMovies([data])

        return data
      } catch (error) {
        this.error = error.response?.data?.message || 'Failed to fetch movie'
        throw error
      } finally {
        this.loading = false
      }
    },

    async fetchMoviesByUser(username, page = 0, size = 10) {
      this.loading = true
      this.error = null
      try {
        const { data } = await api.get(`/movies/user/${username}`, {
          params: {
            page,
            size,
            sort: this.sort,
          },
        })
        this.userMovies = data.content || []
        this.userCurrentPage = data.number || 0
        this.userTotalPages = data.totalPages || 0
        this.userTotalElements = data.totalElements || 0

        // Fetch user votes for the visible user movies
        if (this.userMovies.length > 0) {
          await this.fetchUserVotesForMovies(this.userMovies)
        }

        return data
      } catch (error) {
        this.error = error.response?.data?.message || 'Failed to fetch user movies'
        throw error
      } finally {
        this.loading = false
      }
    },

    async fetchUserVotesForMovies(movies) {
      try {
        const movieIds = movies.map((movie) => movie.id)

        if (movieIds.length === 0) return

        const { data } = await api.post('/votes/user-votes', movieIds)

        const votesMap = new Map()
        Object.entries(data).forEach(([movieId, voteType]) => {
          votesMap.set(parseInt(movieId), voteType)
        })

        this.userVotes = new Map([...this.userVotes, ...votesMap])

        return votesMap
      } catch (error) {
        console.error('Failed to fetch user votes:', error)
      }
    },

    async addMovie(title, description) {
      this.loading = true
      this.error = null
      try {
        const { data } = await api.post('/movies', { title, description })
        this.movies.unshift(data)
        return data
      } catch (error) {
        this.error = error.response?.data?.message || 'Failed to add movie. Are you logged in?'
        throw error
      } finally {
        this.loading = false
      }
    },

    async vote(movieId, type) {
      this.voting = true
      this.error = null
      try {
        const { data } = await api.post(`/movies/${movieId}/vote`, null, {
          params: { type },
        })

        // Update user's vote
        this.userVotes.set(movieId, type)

        // Update movie in-place (preferred)
        const updateMovieInArray = (array) => {
          const movie = array.find((m) => m.id === movieId)
          if (movie) {
            // copy all returned fields into existing object
            Object.assign(movie, data)
            return true
          }
          return false
        }

        // try updating in-place, if not found use splice to replace preserving array identity
        if (!updateMovieInArray(this.movies)) {
          const idx = this.movies.findIndex((m) => m.id === movieId)
          if (idx !== -1) this.movies.splice(idx, 1, data)
        }
        if (!updateMovieInArray(this.userMovies)) {
          const idxU = this.userMovies.findIndex((m) => m.id === movieId)
          if (idxU !== -1) this.userMovies.splice(idxU, 1, data)
        }

        if (this.currentMovie?.id === movieId) {
          Object.assign(this.currentMovie, data)
        }

        return data
      } catch (error) {
        this.error = error.response?.data?.message || 'Failed to vote. Are you logged in?'
        throw error
      } finally {
        this.voting = false
      }
    },

    // Pagination actions
    async nextPage() {
      if (this.hasNextPage) {
        await this.fetchMovies(this.currentPage + 1, this.pageSize)
      }
    },

    async prevPage() {
      if (this.hasPrevPage) {
        await this.fetchMovies(this.currentPage - 1, this.pageSize)
      }
    },

    async goToPage(page) {
      if (page >= 0 && page < this.totalPages) {
        await this.fetchMovies(page, this.pageSize)
      }
    },

    async nextUserPage(username) {
      if (this.hasNextUserPage) {
        await this.fetchMoviesByUser(username, this.userCurrentPage + 1, this.pageSize)
      }
    },

    async prevUserPage(username) {
      if (this.hasPrevUserPage) {
        await this.fetchMoviesByUser(username, this.userCurrentPage - 1, this.pageSize)
      }
    },

    async goToUserPage(username, page) {
      if (page >= 0 && page < this.userTotalPages) {
        await this.fetchMoviesByUser(username, page, this.pageSize)
      }
    },

    setPageSize(size) {
      this.pageSize = size
      // Reset to first page when changing page size
      this.currentPage = 0
      this.userCurrentPage = 0
    },

    clearError() {
      this.error = null
    },

    clearUserVotes() {
      this.userVotes.clear()
    },
  },
})

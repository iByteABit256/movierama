import { defineStore } from "pinia";
import api from "../api/api";

export const useMoviesStore = defineStore("movies", {
  state: () => ({
    movies: [],
    currentMovie: null,
    userMovies: [],
    userVotes: new Map(),
    sort: "dateAdded,desc",
    loading: false,
    error: null
  }),

  getters: {
    getUserVote: (state) => (movieId) => {
      return state.userVotes.get(movieId) || null;
    }
  },

  actions: {
    async fetchMovies(page = 0, size = 10) {
      this.loading = true;
      this.error = null;
      try {
        const { data } = await api.get("/movies", {
          params: { 
            page,
            size,
            sort: this.sort 
          }
        });
        this.movies = data.content || [];
        
        // Fetch user votes for the visible movies
        if (this.movies.length > 0) {
          await this.fetchUserVotesForMovies(this.movies);
        }
        
        return data;
      } catch (error) {
        this.error = error.response?.data?.message || "Failed to fetch movies";
        throw error;
      } finally {
        this.loading = false;
      }
    },

    async fetchMovieById(movieId) {
      this.loading = true;
      this.error = null;
      try {
        const { data } = await api.get(`/movies/${movieId}`);
        this.currentMovie = data;
        
        // Fetch user vote for this single movie
        await this.fetchUserVotesForMovies([data]);
        
        return data;
      } catch (error) {
        this.error = error.response?.data?.message || "Failed to fetch movie";
        throw error;
      } finally {
        this.loading = false;
      }
    },

    async fetchMoviesByUser(username, page = 0, size = 10) {
      this.loading = true;
      this.error = null;
      try {
        const { data } = await api.get(`/movies/user/${username}`, {
          params: { 
            page,
            size,
            sort: this.sort 
          }
        });
        this.userMovies = data.content || [];
        
        // Fetch user votes for the visible user movies
        if (this.userMovies.length > 0) {
          await this.fetchUserVotesForMovies(this.userMovies);
        }
        
        return data;
      } catch (error) {
        this.error = error.response?.data?.message || "Failed to fetch user movies";
        throw error;
      } finally {
        this.loading = false;
      }
    },

    async fetchUserVotesForMovies(movies) {
      try {
        const movieIds = movies.map(movie => movie.id);
        
        if (movieIds.length === 0) return;
        
        const { data } = await api.post("/votes/user-votes", movieIds);
        
        const votesMap = new Map();
        Object.entries(data).forEach(([movieId, voteType]) => {
          votesMap.set(parseInt(movieId), voteType);
        });
        
        this.userVotes = new Map([...this.userVotes, ...votesMap]);
        
        return votesMap;
      } catch (error) {
        console.error("Failed to fetch user votes:", error);
      }
    },

    async addMovie(title, description) {
      this.loading = true;
      this.error = null;
      try {
        const { data } = await api.post("/movies", { title, description });
        this.movies.unshift(data);
        return data;
      } catch (error) {
        this.error = error.response?.data?.message || "Failed to add movie";
        throw error;
      } finally {
        this.loading = false;
      }
    },

    async vote(movieId, type) {
      this.loading = true;
      this.error = null;
      try {
        const { data } = await api.post(`/movies/${movieId}/vote`, null, {
          params: { type },
        });
        
        // Update the user's vote in the local state
        this.userVotes.set(movieId, type);
        
        // Update the movie in both movies and userMovies arrays
        const updateMovieInArray = (array) => {
          const index = array.findIndex((m) => m.id === movieId);
          if (index !== -1) array[index] = data;
        };
        
        updateMovieInArray(this.movies);
        updateMovieInArray(this.userMovies);
        
        if (this.currentMovie?.id === movieId) {
          this.currentMovie = data;
        }
        
        return data;
      } catch (error) {
        this.error = error.response?.data?.message || "Failed to vote";
        throw error;
      } finally {
        this.loading = false;
      }
    },

    clearError() {
      this.error = null;
    },
    
    clearUserVotes() {
      this.userVotes.clear();
    }
  },
});

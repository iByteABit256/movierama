import http from 'k6/http';
import { sleep, check, group } from 'k6';

export let options = {
  stages: [
    { duration: '10s', target: 50 },  // ramp-up
    { duration: '30s', target: 50 },  // steady load
    { duration: '10s', target: 0 },   // ramp-down
  ],
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:9000/api/v1';
const TOKEN = __ENV.TOKEN || ''; // Pass via env var, e.g., TOKEN="..." k6 run load_test.js

// Reusable headers
function getHeaders() {
  return TOKEN
    ? { headers: { Authorization: `Bearer ${TOKEN}`, 'Content-Type': 'application/json' } }
    : { headers: { 'Content-Type': 'application/json' } };
}

export default function () {
  // Group 1: Fetch all movies
  group('GET /movies', function () {
    let res = http.get(`${BASE_URL}/movies`, getHeaders());
    check(res, { 'status 200': (r) => r.status === 200 });
    sleep(1);
  });

  // Group 2: Like a movie
  group('POST /movies/{id}/vote', function () {
    const movieId = 20;
    const res = http.post(`${BASE_URL}/movies/${movieId}/vote?type=LIKE`, null, getHeaders());
    check(res, { 'status 200 or 201': (r) => r.status === 200 || r.status === 201 });
    sleep(1);
  });

  // Group 3: Fetch all movies by user
  group('GET /movies/user/{username}', function () {
    const username = 'user1';
    const res = http.get(`${BASE_URL}/movies/user/${username}`, getHeaders());
    check(res, { 'status 200': (r) => r.status === 200 });
    sleep(1);
  });

  // Group 4: Update a movie
  group('PUT /movies/{id}', function () {
    const payload = JSON.stringify({
      title: 'Marvel',
      description: 'No one has seen anything like this before',
    });

    const movieId = 1;
    const res = http.put(`${BASE_URL}/movies/${movieId}`, payload, getHeaders());
    check(res, { 'status 200': (r) => r.status === 200 });
    sleep(1);
  });
}

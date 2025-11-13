import http from 'k6/http';
import { sleep, check } from 'k6';

export let options = {
  stages: [
    { duration: '10s', target: 50 },   // ramp-up to 50 users
    { duration: '30s', target: 50 },   // steady load
    { duration: '10s', target: 0 },    // ramp-down
  ],
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:9000/api/v1/movies';

export default function () {
  let res = http.get(BASE_URL);
  check(res, {
    'status is 200': (r) => r.status === 200,
  });
  sleep(1);
}


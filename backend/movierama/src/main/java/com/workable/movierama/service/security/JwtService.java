package com.workable.movierama.service.security;

import io.jsonwebtoken.*;
import io.jsonwebtoken.security.Keys;
import java.security.Key;
import java.util.Date;
import org.springframework.stereotype.Service;

@Service
public class JwtService {

  private static final String SECRET =
      "your-super-secret-jwt-key-that-should-be-very-long"; // TODO: move to env variable
  private static final long EXPIRATION_MS = 24 * 60 * 60 * 1000; // 24 hours TODO: make configurable

  private Key getSigningKey() {
    return Keys.hmacShaKeyFor(SECRET.getBytes());
  }

  public String generateToken(String username) {
    return Jwts.builder()
        .setSubject(username)
        .setIssuedAt(new Date())
        .setExpiration(new Date(System.currentTimeMillis() + EXPIRATION_MS))
        .signWith(getSigningKey(), SignatureAlgorithm.HS256)
        .compact();
  }

  public String extractUsername(String token) {
    return Jwts.parserBuilder()
        .setSigningKey(getSigningKey())
        .build()
        .parseClaimsJws(token)
        .getBody()
        .getSubject();
  }

  public boolean isTokenValid(String token, String username) {
    try {
      String extractedUsername = extractUsername(token);
      return extractedUsername.equals(username) && !isExpired(token);
    } catch (JwtException e) {
      return false;
    }
  }

  private boolean isExpired(String token) {
    Date expiration =
        Jwts.parserBuilder()
            .setSigningKey(getSigningKey())
            .build()
            .parseClaimsJws(token)
            .getBody()
            .getExpiration();
    return expiration.before(new Date());
  }
}

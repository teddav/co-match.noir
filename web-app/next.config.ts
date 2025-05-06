import type { NextConfig } from "next";

const isProduction = process.env.NODE_ENV === "production";

const nextConfig: NextConfig = {
  reactStrictMode: true,
  env: {
    NEXT_PUBLIC_API_URL: isProduction ? "https://94.130.176.20.sslip.io:8000" : "http://localhost:8000",
  },
};

export default nextConfig;

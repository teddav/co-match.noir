import { UserPreferences } from "./Preferences";

const API_URL = process.env.NEXT_PUBLIC_API_URL;

export const getMatches = async (token: string) => {
  const response = await fetch(`${API_URL}/matches`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
  return response.json();
};

export const postMatches = async (token: string) => {
  const response = await fetch(`${API_URL}/matches`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
  return response.json();
};

export const postShares = async (urlParams: URLSearchParams, formData: FormData) => {
  const response = await fetch(`${API_URL}/upload?` + urlParams.toString(), {
    method: "POST",
    body: formData,
  });
  return response.json();
};

export const splitPreferences = async (preferences: UserPreferences) => {
  const res = await fetch(`${API_URL}/split`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ user1: preferences }),
  });
  return res.json();
};

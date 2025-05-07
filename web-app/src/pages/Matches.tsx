import { useState } from "react";
import { getMatches, postMatches } from "./api";

type Match = string;

export default function Matches() {
  const [matches, setMatches] = useState<Match[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchMatches = async () => {
    const token = localStorage.getItem("co-match-token");
    if (!token) {
      setError("No token found. Please try uploading your shares again.");
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const data = await getMatches(token);
      console.log("DATA", data);
      setMatches(data.matches);
    } catch (error) {
      console.error(error);
    }
  };

  const computeNewMatches = async () => {
    console.log("COMPUTING NEW MATCHES");
    const token = localStorage.getItem("co-match-token");
    if (!token) {
      setError("No token found. Please try uploading your shares again.");
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const data = await postMatches(token);
      console.log("DATA", data);
      if (data == "ok") {
        fetchMatches();
      } else {
        setError("Failed to get matches. Please try again.");
      }
    } catch (error) {
      console.error(error);
      setError("Failed to get matches. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="bg-white/90 backdrop-blur-md rounded-2xl shadow-lg p-8 border border-pink-100">
        <h2 className="text-3xl font-bold bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent mb-6">
          Loading Matches... 💫
        </h2>
        <div className="animate-pulse space-y-4">
          <div className="h-4 bg-gradient-to-r from-pink-100 to-purple-100 rounded-xl w-3/4"></div>
          <div className="h-4 bg-gradient-to-r from-pink-100 to-purple-100 rounded-xl"></div>
          <div className="h-4 bg-gradient-to-r from-pink-100 to-purple-100 rounded-xl w-5/6"></div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white/90 backdrop-blur-md rounded-2xl shadow-lg p-8 border border-pink-100">
        <h2 className="text-3xl font-bold bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent mb-6">Oops! 💔</h2>
        <div className="p-4 bg-red-50 text-red-700 rounded-xl border border-red-100 mb-6">{error}</div>
        <button
          onClick={computeNewMatches}
          className="w-full py-3 px-6 rounded-xl transition-all duration-300 bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
        >
          Try Again
        </button>
      </div>
    );
  }

  if (matches.length === 0) {
    return (
      <div className="bg-white/90 backdrop-blur-md rounded-2xl shadow-lg p-8 border border-pink-100">
        <h2 className="text-3xl font-bold bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent mb-6">
          No Matches Yet 💭
        </h2>
        <p className="text-gray-600 mb-8">Click the button below to check for matches!</p>
        <button
          onClick={computeNewMatches}
          className="w-full py-3 px-6 rounded-xl transition-all duration-300 bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
        >
          Check for Matches
        </button>
      </div>
    );
  }

  console.log("MATCHES", matches);

  return (
    <div className="bg-white/90 backdrop-blur-md rounded-2xl shadow-lg p-8 border border-pink-100">
      <div className="flex justify-between items-center mb-8">
        <h2 className="text-3xl font-bold bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent">Your Matches 💖</h2>
        <button
          onClick={computeNewMatches}
          className="px-6 py-3 rounded-xl transition-all duration-300 bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
        >
          Refresh Matches
        </button>
      </div>
      <div className="space-y-4">
        {matches.map((match_) => {
          const handle = match_.startsWith("@") ? match_.slice(1) : match_;

          return (
            <div key={match_} className="border border-gray-200 rounded-xl p-6 hover:border-purple-300 transition-all hover:shadow-md">
              <div className="flex items-center justify-between">
                <div>
                  <h3 className="text-xl font-semibold text-gray-900">@{handle}</h3>
                  <p className="text-sm text-gray-500">You have a match! 🎉</p>
                </div>
                <div className="flex gap-3">
                  <a
                    href={`https://twitter.com/${handle}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="px-4 py-2 border border-purple-600 text-purple-700 rounded-xl hover:bg-purple-50 transition-all text-sm font-medium"
                  >
                    View Profile →
                  </a>
                  <a
                    href={`https://twitter.com/intent/tweet?text=I'm a match with @${handle} on Co-Match!!! ❤️ %0APrivate dating made possible thanks to Noir and coSNARKs🪄🥳 %0AThanks @TACEO_IO @NoirLang @0xteddav %0A https://co-match.vercel.app`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="px-4 py-2 rounded-xl transition-all duration-300 bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5 text-sm font-medium"
                  >
                    Send a love message
                  </a>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

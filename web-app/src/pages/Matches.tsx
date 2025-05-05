import { useState, useEffect } from "react";

interface Match {
  id: string;
  twitter_handle: string;
}

export default function Matches() {
  const [matches, setMatches] = useState<Match[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchMatches = async () => {
      const token = localStorage.getItem("co-match-token");
      if (!token) {
        setError("No token found. Please try uploading your shares again.");
        setLoading(false);
        return;
      }

      try {
        const response = await fetch("http://localhost:8000/", {
          method: "GET",
          headers: {
            Authorization: `Bearer ${token}`,
          },
        });

        const data = await response.json();
        setMatches(data);
      } catch (error) {
        console.error(error);
        setError("Failed to get matches. Please try again.");
      } finally {
        setLoading(false);
      }
    };

    fetchMatches();
  }, []);

  if (loading) {
    return (
      <div className="bg-white rounded-lg shadow-lg p-6">
        <h2 className="text-2xl font-semibold text-gray-800 mb-6">Loading Matches...</h2>
        <div className="animate-pulse space-y-4">
          <div className="h-4 bg-gray-200 rounded w-3/4"></div>
          <div className="h-4 bg-gray-200 rounded"></div>
          <div className="h-4 bg-gray-200 rounded w-5/6"></div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white rounded-lg shadow-lg p-6">
        <h2 className="text-2xl font-semibold text-gray-800 mb-6">Error</h2>
        <div className="p-4 bg-red-50 text-red-700 rounded-md">{error}</div>
      </div>
    );
  }

  if (matches.length === 0) {
    return (
      <div className="bg-white rounded-lg shadow-lg p-6">
        <h2 className="text-2xl font-semibold text-gray-800 mb-6">No Matches Yet</h2>
        <p className="text-gray-600">We haven&apos;t found any matches for you yet. Don&apos;t worry, we&apos;ll notify you when we do!</p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-lg p-6">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6">Your Matches</h2>
      <div className="space-y-4">
        {matches.map((match_) => (
          <div key={match_.id} className="border rounded-lg p-4 hover:bg-gray-50">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-lg font-medium text-gray-900">@{match_.twitter_handle}</h3>
                <p className="text-sm text-gray-500">You have a match! 🎉</p>
              </div>
              <a
                href={`https://twitter.com/${match_.twitter_handle}`}
                target="_blank"
                rel="noopener noreferrer"
                className="text-purple-600 hover:text-purple-700"
              >
                View Profile →
              </a>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

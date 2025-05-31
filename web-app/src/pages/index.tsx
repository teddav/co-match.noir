import { useState, useEffect } from "react";
import Preferences, { UserPreferences } from "./Preferences";
import Shares from "./Shares";
import Matches from "./Matches";

type View = "preferences" | "shares" | "matches";

const CIRCUIT_VERSION = 1;

export default function Home() {
  const [view, setView] = useState<View>("preferences");
  const [preferences, setPreferences] = useState<UserPreferences | null>(null);
  const [showPreferences, setShowPreferences] = useState(true);

  useEffect(() => {
    const storedVersion = localStorage.getItem("co-match-circuit-version");
    if (storedVersion !== String(CIRCUIT_VERSION)) {
      // Version changed or not set, reset everything
      localStorage.clear();
      localStorage.setItem("co-match-circuit-version", String(CIRCUIT_VERSION));
      setView("preferences");
      setPreferences(null);
      setShowPreferences(false);
      window.location.reload();
      return;
    }
  }, []);

  useEffect(() => {
    const savedPreferences = localStorage.getItem("co-match-preferences");
    const token = localStorage.getItem("co-match-token");

    if (savedPreferences && token) {
      setView("matches");
      setPreferences(JSON.parse(savedPreferences));
      setShowPreferences(false);
    } else if (savedPreferences) {
      setView("shares");
      setPreferences(JSON.parse(savedPreferences));
    }
  }, []);

  const handlePreferencesSubmit = (newPreferences: UserPreferences) => {
    setPreferences(newPreferences);
    setView("shares");
    setShowPreferences(false);
  };

  const handleMatchesSubmit = () => {
    setView("matches");
  };

  const renderContent = () => {
    switch (view) {
      case "preferences":
        return <></>;
      case "shares":
        return preferences ? <Shares preferences={preferences} getMatches={handleMatchesSubmit} /> : null;
      case "matches":
        return <Matches />;
      default:
        return <Preferences onSubmit={handlePreferencesSubmit} />;
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-pink-100 via-purple-50 to-pink-50 relative overflow-hidden">
      {/* Decorative elements */}
      <div className="absolute top-0 left-0 w-full h-full overflow-hidden pointer-events-none">
        <div className="absolute top-20 left-10 w-32 h-32 bg-pink-200 rounded-full opacity-20 blur-2xl"></div>
        <div className="absolute bottom-20 right-10 w-40 h-40 bg-purple-200 rounded-full opacity-20 blur-2xl"></div>
      </div>

      <div className="max-w-4xl mx-auto px-4 py-8 relative">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-5xl font-bold bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent">
            Co-Match Dating 🌶️
          </h1>
        </div>

        <div className="bg-yellow-50 border border-yellow-200 rounded-xl p-6 mb-8">
          <h3 className="text-xl font-semibold text-yellow-800 mb-2">⚠️ MPC Server Status</h3>
          <p className="text-yellow-700">
            The app is currently not functional as the MPC server is not running. To try out the app, you&apos;ll need to run your own MPC
            server. Check out the{" "}
            <a
              href="https://github.com/teddav/co-match.nr"
              target="_blank"
              rel="noopener noreferrer"
              className="text-yellow-800 underline hover:text-yellow-900"
            >
              source code
            </a>{" "}
            for instructions on setting up the server.
          </p>
        </div>

        <div className="bg-white/90 backdrop-blur-md rounded-2xl p-8 mb-8 shadow-lg border border-pink-100">
          <h2 className="text-2xl font-semibold text-purple-700 mb-4">How It Works</h2>
          <p className="text-gray-700 mb-4">
            Co-Match is a privacy-focused dating app that uses <b>zero-knowledge proofs</b> and <b>multiparty computation</b> to match you
            with potential partners without revealing your personal information. Here&apos;s how it works:
          </p>
          <ol className="list-decimal list-inside space-y-2 text-gray-600">
            <li>Set your profile and preferences (you can select up to 3 interests)</li>
            <li>Our system will find potential matches based on your criteria</li>
            <li>If there&apos;s a mutual match, you&apos;ll both be notified</li>
            <li>Your personal data remains private throughout the entire process</li>
          </ol>
          <p className="text-gray-700 mt-6 p-4 bg-gradient-to-r from-pink-50 to-purple-50 rounded-xl border border-pink-100">
            This is a proof of concept built with{" "}
            <a
              href="https://noir-lang.org"
              target="_blank"
              rel="noopener noreferrer"
              className="text-purple-600 hover:text-purple-700 font-medium"
            >
              Noir
            </a>{" "}
            and{" "}
            <a
              href="https://taceo.io/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-purple-600 hover:text-purple-700 font-medium"
            >
              co-snarks
            </a>
            , demonstrating how zero-knowledge proofs and multiparty computation can be used to create truly private dating applications.
            <br />
            Check out the{" "}
            <a
              href="https://github.com/teddav/co-match.nr"
              target="_blank"
              rel="noopener noreferrer"
              className="text-purple-600 hover:text-purple-700 font-medium"
            >
              source code
            </a>{" "}
            to learn more about the implementation! 🚀
          </p>
        </div>

        {view !== "preferences" && (
          <div className="flex justify-center my-8">
            <button
              onClick={() => setShowPreferences(!showPreferences)}
              className="px-6 py-3 bg-gradient-to-r from-pink-500 to-purple-500 text-white rounded-full hover:from-pink-600 hover:to-purple-600 transition-all duration-300 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
            >
              {showPreferences ? "Hide Preferences" : "View Preferences"}
            </button>
          </div>
        )}

        <div className="grid gap-8">
          {showPreferences && (
            <div className="space-y-8">
              <Preferences onSubmit={handlePreferencesSubmit} />
            </div>
          )}
          {renderContent()}
        </div>
      </div>
    </div>
  );
}

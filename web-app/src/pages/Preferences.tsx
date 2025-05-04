import { useState } from "react";

// const data = {
//   user1: {
//     age: 30,
//     gender: 0,
//     id: "0x1fed07ad686a727dfc33b91206d526e61f519dca9c5054ae729231c201717633",
//     id_nullifier: 12345,
//     interests: [5, 8, 10],
//     region: 1,
//     preferences: {
//       age_max: 35,
//       age_min: 25,
//       gender: 1,
//     },
//   },
//   user2: {
//     age: 32,
//     gender: 1,
//     id: "0x16e31ced6c74696a601f45f1bb2b9833380d51348fe89644360d0e5abeaf244a",
//     id_nullifier: 67890,
//     interests: [10, 20, 30],
//     region: 1,
//     preferences: {
//       age_max: 35,
//       age_min: 25,
//       gender: 1,
//     },
//   },
// };
interface UserPreferences {
  id: string;
  id_nullifier: number;

  age: number;
  gender: number;
  interests: number[];
  region: number;
  preferences: {
    age_min: number;
    age_max: number;
    gender: number;
  };
}

interface PreferencesProps {
  onSubmit: (data: { shares: Uint8Array[] }) => void;
}

const INTERESTS = [
  { id: 1, label: "Travel" },
  { id: 2, label: "Music" },
  { id: 3, label: "Cooking" },
  { id: 4, label: "Sports" },
  { id: 5, label: "Reading" },
  { id: 6, label: "Movies" },
  { id: 7, label: "Fitness" },
  { id: 8, label: "Art" },
  { id: 9, label: "Photography" },
  { id: 10, label: "Technology" },
];

export default function Preferences({ onSubmit }: PreferencesProps) {
  const [preferences, setPreferences] = useState<UserPreferences>({
    id: "0x1fed07ad686a727dfc33b91206d526e61f519dca9c5054ae729231c201717633",
    id_nullifier: 12345,

    age: 25,
    gender: 0,
    interests: [],
    region: 1,
    preferences: {
      age_min: 20,
      age_max: 35,
      gender: 1,
    },
  });

  const handleInterestChange = (interestId: number) => {
    setPreferences((prev) => ({
      ...prev,
      interests: prev.interests.includes(interestId) ? prev.interests.filter((id) => id !== interestId) : [...prev.interests, interestId],
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const res = await fetch("/api/split", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ user1: preferences }),
      });
      const response_data = await res.json();
      console.log("response_data", response_data);

      // Convert the response data to Uint8Array shares
      const shares = response_data.shares.map((share: string) => new Uint8Array(share.split(",").map(Number)));

      onSubmit({ shares });
    } catch (error) {
      console.log(error);
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-lg p-6">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6">Your Preferences</h2>
      <form onSubmit={handleSubmit} className="space-y-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Your Age</label>
            <input
              type="number"
              value={preferences.age}
              onChange={(e) => setPreferences({ ...preferences, age: parseInt(e.target.value) })}
              className="w-full px-3 py-2 border border-gray-300 rounded-md"
              min="18"
              max="100"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Your Gender</label>
            <select
              value={preferences.gender}
              onChange={(e) => setPreferences({ ...preferences, gender: parseInt(e.target.value) })}
              className="w-full px-3 py-2 border border-gray-300 rounded-md"
            >
              <option value={0}>Female</option>
              <option value={1}>Male</option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Preferred Age Range</label>
            <div className="flex gap-4">
              <input
                type="number"
                value={preferences.preferences.age_min}
                onChange={(e) =>
                  setPreferences({
                    ...preferences,
                    preferences: { ...preferences.preferences, age_min: parseInt(e.target.value) },
                  })
                }
                className="w-full px-3 py-2 border border-gray-300 rounded-md"
                min="18"
                max="100"
                placeholder="Min"
              />
              <input
                type="number"
                value={preferences.preferences.age_max}
                onChange={(e) =>
                  setPreferences({
                    ...preferences,
                    preferences: { ...preferences.preferences, age_max: parseInt(e.target.value) },
                  })
                }
                className="w-full px-3 py-2 border border-gray-300 rounded-md"
                min="18"
                max="100"
                placeholder="Max"
              />
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Preferred Gender</label>
            <select
              value={preferences.preferences.gender}
              onChange={(e) =>
                setPreferences({
                  ...preferences,
                  preferences: { ...preferences.preferences, gender: parseInt(e.target.value) },
                })
              }
              className="w-full px-3 py-2 border border-gray-300 rounded-md"
            >
              <option value={0}>Female</option>
              <option value={1}>Male</option>
              <option value={2}>Any</option>
            </select>
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-3">
            Your Interests (Select up to 3)
            <span className="ml-2 text-sm text-gray-500">{preferences.interests.length}/3 selected</span>
          </label>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-3">
            {INTERESTS.map((interest) => (
              <label
                key={interest.id}
                className={`flex items-center space-x-2 p-2 border rounded-md cursor-pointer ${
                  !preferences.interests.includes(interest.id) && preferences.interests.length >= 3
                    ? "opacity-50 cursor-not-allowed"
                    : "hover:bg-gray-50"
                }`}
              >
                <input
                  type="checkbox"
                  checked={preferences.interests.includes(interest.id)}
                  onChange={() => handleInterestChange(interest.id)}
                  disabled={!preferences.interests.includes(interest.id) && preferences.interests.length >= 3}
                  className="rounded text-purple-600 focus:ring-purple-500"
                />
                <span className="text-sm text-gray-700">{interest.label}</span>
              </label>
            ))}
          </div>
        </div>

        <button type="submit" className="w-full bg-purple-600 text-white py-2 px-4 rounded-md hover:bg-purple-700 transition-colors">
          Save Preferences
        </button>
      </form>
    </div>
  );
}

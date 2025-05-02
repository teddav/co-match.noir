async function fetchData() {
  const data = {
    user1: {
      age: 30,
      gender: 0,
      id: "0x1fed07ad686a727dfc33b91206d526e61f519dca9c5054ae729231c201717633",
      id_nullifier: 12345,
      interests: [5, 8, 10],
      region: 1,
      preferences: {
        age_max: 35,
        age_min: 25,
        gender: 1,
      },
    },
    user2: {
      age: 32,
      gender: 1,
      id: "0x16e31ced6c74696a601f45f1bb2b9833380d51348fe89644360d0e5abeaf244a",
      id_nullifier: 67890,
      interests: [10, 20, 30],
      region: 1,
      preferences: {
        age_max: 35,
        age_min: 25,
        gender: 1,
      },
    },
  };

  try {
    const res = await fetch("/api/split", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });
    const response_data = await res.json();
    console.log("response_data", response_data);
  } catch (error) {
    console.log(error);
  }
}

export default function Preferences() {
  return (
    <div>
      <div className="flex gap-4 items-center flex-col sm:flex-row">main</div>
      <button onClick={fetchData}>Fetch Data</button>
    </div>
  );
}

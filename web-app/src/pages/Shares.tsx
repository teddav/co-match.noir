async function uploadShares() {
  const uint8 = new Uint8Array([72, 101, 108, 108, 111]);
  const blob = new Blob([uint8], { type: "application/octet-stream" });

  const formData = new FormData();
  formData.append("file1", blob);
  formData.append("file2", blob);
  formData.append("file3", blob);
  formData.append("file4", blob);
  formData.append("file5", blob);
  formData.append("file6", blob);

  const params = new URLSearchParams({
    twitter_handle: "@HELLO",
  });

  const response = await fetch("http://localhost:8000/upload?" + params.toString(), {
    method: "POST",
    body: formData,
  });
  const data = await response.json();
  console.log(data);
}

export default function Shares() {
  return (
    <div>
      <h1>Shares</h1>
      <button onClick={uploadShares}>Upload Shares</button>
    </div>
  );
}

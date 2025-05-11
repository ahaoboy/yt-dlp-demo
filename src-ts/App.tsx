import './App.css'
import useSWR from 'swr'
import { getVersion } from './api'

function App() {
  const { data, error, isLoading } = useSWR('/api/user', getVersion)
  if (error) return <div>failed to load</div>
  if (isLoading) return <div>loading...</div>
  return <div>
    <div>ytdlp: {data?.ytdlp}</div>
    <div>ffmpeg: {data?.ffmpeg.split("\n")[0]}</div>
  </div>
}

export default App

export type JobStatus = "RENDERING"

export interface Job {
  id: string,
  thumbnail: string,
  status: JobStatus,
  render: RenderStatus
}

export interface RenderStatus {
  frameStart: number,
  frameEnd: number,
  framesRendered: number,
  averageFrameTime: number
}

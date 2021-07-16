export interface User {
  id: string;
  // eslint-disable-next-line camelcase
  github_id: number;
  username: string;
  name: string;
  email: null;
  // eslint-disable-next-line camelcase
  avatar_url: string;
  bio: string;
  created: Date;
  role: string;
}

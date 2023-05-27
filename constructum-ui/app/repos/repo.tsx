import { UUID } from "crypto";

export default interface Repo {
    id: UUID,
    name: string,
    description: string,
    html_url: string,
    ssh_url: string,
    owner: GiteaUser,
    is_registered: boolean
}

export interface GiteaUser {
    id: number,
    login: string,
}
import { UUID } from "crypto";

export default interface Repo {
    id: UUID,
    name: string,
    description: string,
    html_url: string,
    ssh_url: string,
    owner: string,
    is_registered: boolean
}
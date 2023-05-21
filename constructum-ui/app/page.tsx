import HeaderComponent from "./header";
import MainBodyComponent from "./main_body";
import RepoPage from "./repos/page";

export default function Home() {
  return (
    <>
      <HeaderComponent locationName="Dashboard" />
      <MainBodyComponent>
        {/* @ts-expect-error Async Server Component */}
        <RepoPage/>
      </MainBodyComponent>
    </>
  );
}

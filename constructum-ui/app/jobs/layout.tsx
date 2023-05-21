import HeaderComponent from "../header";
import MainBodyComponent from "../main_body";

export default async function JobsLayout({
    children,
  }: {
    children: React.ReactNode;
  }) {
  
    return (
      <>
        <HeaderComponent locationName="Job"/>
        <MainBodyComponent>
            {children}
        </MainBodyComponent>
      </>
    );
  }
  
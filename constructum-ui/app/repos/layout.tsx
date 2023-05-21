import HeaderComponent from "../header";
import MainBodyComponent from "../main_body";

export default async function ReposLayout({
    children,
  }: {
    children: React.ReactNode;
  }) {
  
    return (
      <>
        <HeaderComponent locationName="Repositories"/>
        <MainBodyComponent>
            {children}
        </MainBodyComponent>
      </>
    );
  }
  
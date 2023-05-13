import "./globals.css";
import { Inter } from "next/font/google";
import HeadbarComponent from "./headbar";
import { getServerSession } from "next-auth/next";
import { authOptions } from "@/pages/api/auth/[...nextauth]";
import SignIn from "./signin";
import UserInformation from "./user_info";
import AuthContext from "./auth_context";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "Constructum",
  description: "Your favorite CI/CD tool",
};

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await getServerSession(authOptions);

  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="min-h-full">
          {session?.user ? (
            <AuthContext>
              <HeadbarComponent/>
              <header className="bg-white shadow">
                <div className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
                  <h1 className="text-3xl font-bold tracking-tight text-gray-900">
                    Dashboard
                  </h1>
                </div>
              </header>
              <main>
                <div className="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
                  {children}
                </div>
              </main>
            </AuthContext>
          ) : (
            <SignIn />
          )}
        </div>
      </body>
    </html>
  );
}

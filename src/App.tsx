import { useState } from "react";
import { SearchView } from "./components/SearchView";
import { BookDetail } from "./components/BookDetail";
import { BookInfo } from "./lib/api";
import "./App.css";

function App() {
  const [selectedBook, setSelectedBook] = useState<BookInfo | null>(null);

  return (
    <div className="app">
      {selectedBook ? (
        <BookDetail book={selectedBook} onBack={() => setSelectedBook(null)} />
      ) : (
        <SearchView onSelectBook={setSelectedBook} />
      )}
    </div>
  );
}

export default App;

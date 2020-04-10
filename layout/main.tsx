import React from 'react'

const Layout: React.FC = (props) => {
  return <><div className="py-24 flex flex-col items-center">
    <h1 className="font-serif text-black text-5xl font-bold mt-5 text-center">Sam Wight</h1>
    <p className="max-w-lg text-black text-xl font-serif text-center">Developer. CS Student at UA. President of Blueprint at UA.</p>
  </div>{props.children}</>
}

export default Layout
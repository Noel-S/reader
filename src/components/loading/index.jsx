import React from "react";
import LoadingSVG from "../../assets/loading.svg";

const Loading = () => {

    // useEffect(() => {
    //     // if (action == 'first')
    //     // check query params to decide if can open a window 
    //     // invoke('open_projects');
    // }, []);

    return (
        <div className="container" style={{ justifyContent: 'center', alignItems: 'center' }}>
          <img src={LoadingSVG} alt="loading" height={200} width={200} />
        </div>
    );
};

export default Loading;
import React from 'react';
// import Surreal from 'surrealdb.js'

import style from './style.module.css';

import chartSVG from '../../assets/icons/chart.svg';
import chartBoldSVG from '../../assets/icons/bold/chart.svg';
import categorySVG from '../../assets/icons/category.svg';
import categoryBoldSVG from '../../assets/icons/bold/category.svg';
import discoverySVG from '../../assets/icons/discovery.svg';
import discoveryBoldSVG from '../../assets/icons/bold/discovery.svg';
import settingSVG from '../../assets/icons/setting.svg';
import settingBoldSVG from '../../assets/icons/bold/setting.svg';
import editSVG from '../../assets/icons/edit.svg';
import editBoldSVG from '../../assets/icons/bold/edit.svg';

const Sidebar = () => {
    // const readFromDB = async (evt) => {
    //     evt.preventDefault();
    //     const db = new Surreal('http://127.0.0.1:8000/rpc');
    //     await db.signin({
    //         user: 'root',
    //         pass: 'root'
    //     });
    //     await db.use('main', 'elections');
    //     const result = await db.query('INFO FOR DB;');
    //     console.log(result);
    // };
    return (
        <nav className={style.sidebar}>
            <p className={style.sidebarTitle}>Rea<span>der</span></p>
            {/* <p className={style.tagLine}>by yocusoft</p> */}
            <section className={style.sidebarOptions}>
                <button className={style.sidebarButtonActive}><img src={chartBoldSVG} alt="chart" />Dashboard</button>
                <button><img src={editSVG} alt="edit" />Query</button>
                <button><img src={categorySVG} alt="category" />Option 1</button>
                <button><img src={discoverySVG} alt="discovery" />Option 2</button>
                <button><img src={settingSVG} alt="setting" />Settings</button>
            </section>
        </nav>
    );
};

export default Sidebar;

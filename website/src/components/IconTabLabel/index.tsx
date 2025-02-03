import styles from "./styles.module.scss";

export default (props: { icon: React.ReactNode; title: string }) => {
  const { icon, title } = props;

  return (
    <div className={styles.tabItemWithIcon}>
      {icon}
      {title}
    </div>
  );
};

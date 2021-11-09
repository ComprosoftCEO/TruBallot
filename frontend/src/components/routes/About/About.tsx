import { Container, Divider, Header, Segment, Image, List } from 'semantic-ui-react';
import { DashboardMenu, TransitionList } from 'components/shared';

export const About = () => (
  <>
    <DashboardMenu />

    <Container text style={{ marginTop: '8em' }} textAlign="center">
      <TransitionList animation="fade down" duration={500} totalDuration={300}>
        <Image src="/truballot-logo.svg" size="big" centered />

        <div style={{ marginTop: 50 }}>
          <Header size="large">About:</Header>
          <Divider />

          <Segment raised textAlign="left">
            <p>
              The voting protocol behind TruBallot comes from a
              <a href="https://www.mdpi.com/2410-387X/1/2/13"> 2017 paper published by Zou et al. </a>
              The protocol provides the following advantages over other voting systems:
            </p>

            <List bulleted>
              <List.Item>
                Lightweight ballot generation and tallying, which requires only modular arithmetic and multiplication
              </List.Item>
              <List.Item>All encrypted ballots are publicly visible and verifiable</List.Item>
              <List.Item>
                The entire voting process is transparent, and voters participate in verification and tallying
              </List.Item>
              <List.Item>Vote-casting assurance: All ballots are publicly visible</List.Item>
              <List.Item>Vote-tallying assurance: Anyone can recompute the final voting vector</List.Item>
            </List>

            <p>
              The TruBallot frontend, backend, collectors, and notification server have been implemented by Bryan
              McClain for his senior capstone project. Bryan worked with Dr. Zou (capstone advisor) during this project
              to ensure the math was implemented correctly. The frontend is written in TypeScript using
              <a href="https://reactjs.org/"> React</a>. The backend, collectors, and notification server are written in
              Rust using <a href="https://actix.rs/">Actix Web</a> and <a href="https://diesel.rs/"> Diesel </a>
              frameworks. The TruBallot icon and logo were designed by Nathan McClain.
            </p>

            <p>
              Please see the <a href="https://www.mdpi.com/2410-387X/1/2/13">original paper</a> for details on the
              underlying protocol.
            </p>
          </Segment>
        </div>
      </TransitionList>
    </Container>
  </>
);
